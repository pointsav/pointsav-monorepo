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
  default = "us-west1-b"
}

variable "vllm_port" {
  type    = number
  default = 8000
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
    stack = "vllm"
    role  = "yoyo-tier-b"
  }
  ssh_username        = "packer"
  on_host_maintenance = "TERMINATE"
  accelerators {
    type  = "nvidia-l4"
    count = 1
  }
}

build {
  sources = ["source.googlecompute.yoyo"]

  provisioner "file" {
    source      = "scripts/vllm.service"
    destination = "/tmp/vllm.service"
  }

  provisioner "file" {
    source      = "scripts/nginx-yoyo.conf"
    destination = "/tmp/nginx-yoyo.conf"
  }

  provisioner "shell" {
    script = "scripts/provision.sh"
    environment_vars = [
      "DEBIAN_FRONTEND=noninteractive",
      "VLLM_PORT=${var.vllm_port}",
    ]
  }
}
