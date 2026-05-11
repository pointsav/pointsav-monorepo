packer {
  required_plugins {
    googlecompute = {
      source  = "github.com/hashicorp/googlecompute"
      version = "~> 1"
    }
  }
}

variable "project_id" {
  type    = string
  default = "woodfine-node-gcp-free"
}

variable "zone" {
  type    = string
  default = "us-central1-b"
}

variable "vllm_port" {
  type    = number
  default = 8000
}

variable "llama_cpp_ref" {
  type        = string
  default     = "master"
  description = "git ref / commit for llama.cpp clone (set to a pinned SHA for reproducibility)"
}

source "googlecompute" "yoyo" {
  project_id          = var.project_id
  zone                = var.zone
  source_image_family = "ubuntu-2404-lts-amd64"
  machine_type        = "g2-standard-4"
  disk_size           = 50
  image_name          = "slm-yoyo-${formatdate("YYYYMMDD-HHmmss", timestamp())}"
  image_family        = "slm-yoyo"
  image_labels = {
    stack = "vllm-and-training"
    role  = "yoyo-tier-b"
  }
  ssh_username        = "packer"
  on_host_maintenance = "TERMINATE"
  accelerator_type    = "zones/${var.zone}/acceleratorTypes/nvidia-l4"
  accelerator_count   = 1
}

build {
  sources = ["source.googlecompute.yoyo"]

  # systemd units
  provisioner "file" {
    source      = "scripts/vllm.service"
    destination = "/tmp/vllm.service"
  }
  provisioner "file" {
    source      = "scripts/vllm-weights-prep.service"
    destination = "/tmp/vllm-weights-prep.service"
  }
  provisioner "file" {
    source      = "scripts/lora-training.service"
    destination = "/tmp/lora-training.service"
  }
  provisioner "file" {
    source      = "scripts/adapter-publish.service"
    destination = "/tmp/adapter-publish.service"
  }

  # Lifecycle shell scripts
  provisioner "file" {
    source      = "scripts/vllm-weights-prep.sh"
    destination = "/tmp/vllm-weights-prep.sh"
  }
  provisioner "file" {
    source      = "scripts/lora-training.sh"
    destination = "/tmp/lora-training.sh"
  }
  provisioner "file" {
    source      = "scripts/adapter-publish.sh"
    destination = "/tmp/adapter-publish.sh"
  }

  # Nginx TLS reverse proxy config
  provisioner "file" {
    source      = "scripts/nginx-yoyo.conf"
    destination = "/tmp/nginx-yoyo.conf"
  }

  provisioner "shell" {
    script = "scripts/provision.sh"
    environment_vars = [
      "DEBIAN_FRONTEND=noninteractive",
      "VLLM_PORT=${var.vllm_port}",
      "LLAMA_CPP_REF=${var.llama_cpp_ref}",
    ]
  }
}
