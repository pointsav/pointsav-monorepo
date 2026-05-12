terraform {
  required_providers {
    google = {
      source  = "registry.terraform.io/hashicorp/google"
      version = "~> 5"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

# ── Instance Schedule (nightly start + hard stop) ────────────────────────────
# Fires instances.start every night at var.start_time_utc (default 02:00 UTC).
# Fires instances.stop at var.stop_time_utc (default 06:00 UTC) as a hard cap.
# The Doorman idle monitor will typically stop the VM sooner (after 30 min idle
# via SLM_YOYO_IDLE_MINUTES), but the stop schedule is a safety net if the idle
# monitor crashes or loses connectivity.

resource "google_compute_resource_policy" "nightly_start" {
  name   = "${var.instance_name}-nightly-start"
  region = var.region

  instance_schedule_policy {
    time_zone = "UTC"
    vm_start_schedule {
      schedule = "0 ${split(":", var.start_time_utc)[0]} * * *"
    }
    vm_stop_schedule {
      schedule = "0 ${split(":", var.stop_time_utc)[0]} * * *"
    }
  }
}

# ── Persistent data disk (model weights) ─────────────────────────────────────
# 100 GB SSD — survives instances.stop / instances.start cycles.
# Initial population: operator copies OLMo 3 32B-Think Q4 weights via gcloud scp.
# The disk is formatted and mounted by rc.local on first boot.

resource "google_compute_disk" "weights" {
  name = "${var.instance_name}-weights"
  type = "pd-ssd"
  size = 100
  zone = var.zone

  labels = {
    role = "yoyo-weights"
  }
}

# ── Yo-Yo #1 GCE instance ────────────────────────────────────────────────────

resource "google_compute_instance" "yoyo_tier_b" {
  name         = var.instance_name
  machine_type = "g2-standard-4"
  zone         = var.zone

  tags = ["yoyo-tier-b"]

  scheduling {
    # Spot (preemptible) for cost.
    preemptible        = true
    automatic_restart  = false
    on_host_maintenance = "TERMINATE"
  }

  boot_disk {
    initialize_params {
      # Uses the most recent image from the slm-yoyo family built by Packer.
      image = "projects/${var.project_id}/global/images/family/slm-yoyo"
      size  = 50
      type  = "pd-balanced"
    }
  }

  attached_disk {
    source      = google_compute_disk.weights.self_link
    device_name = "yoyo-weights"
    mode        = "READ_WRITE"
  }

  # GCP Instance Schedule — nightly start at var.start_time_utc.
  resource_policies = [google_compute_resource_policy.nightly_start.id]

  # Bearer token stored in metadata; read at boot by rc.local to configure Nginx.
  metadata = {
    "bearer-token" = var.bearer_token
  }

  network_interface {
    network = "default"
    access_config {
      # Ephemeral external IP. The idle monitor stores the IP via env var
      # (SLM_YOYO_ENDPOINT); update local-doorman.env after first tofu apply.
    }
  }

  guest_accelerator {
    type  = "nvidia-l4"
    count = 1
  }

  service_account {
    # Default Compute Engine SA — sufficient for the workspace VM to stop this
    # instance via the ADC token from the metadata server.
    scopes = ["cloud-platform"]
  }
}

# ── Firewall — workspace VM → Yo-Yo port 9443 only ──────────────────────────

resource "google_compute_firewall" "workspace_to_yoyo" {
  name    = "allow-workspace-to-yoyo"
  network = "default"

  allow {
    protocol = "tcp"
    ports    = ["9443"]
  }

  # Only the workspace VM's external IP may reach the Yo-Yo.
  source_ranges = ["${var.workspace_ip}/32"]
  target_tags   = ["yoyo-tier-b"]

  description = "Allow workspace VM to reach Yo-Yo Nginx TLS proxy on port 9443."
}

# ── IAM — idle monitor stop permission ───────────────────────────────────────
# Grants the workspace VM's default Compute Engine SA permission to call
# instances.stop on the Yo-Yo project. The idle monitor uses ADC
# (metadata server token) to authenticate — no key file required.

data "google_compute_default_service_account" "workspace" {
  project = var.project_id
}

resource "google_project_iam_member" "idle_monitor_stop" {
  project = var.project_id
  role    = "roles/compute.instanceAdmin.v1"
  member  = "serviceAccount:${data.google_compute_default_service_account.workspace.email}"
}
