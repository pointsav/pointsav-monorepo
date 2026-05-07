variable "project_id" {
  description = "GCP project ID where Yo-Yo resources are created."
  type        = string
  default     = "woodfine-node-gcp-free"
}

variable "region" {
  description = "GCP region."
  type        = string
  default     = "us-west1"
}

variable "zone" {
  description = "GCP zone within the region."
  type        = string
  default     = "us-west1-b"
}

variable "instance_name" {
  description = "Name of the Yo-Yo GCE instance."
  type        = string
  default     = "yoyo-tier-b-1"
}

variable "start_time_utc" {
  description = "Nightly start time in UTC (HH:MM). GCP Instance Schedule fires instances.start at this time every day."
  type        = string
  default     = "02:00"
}

variable "bearer_token" {
  description = "Static bearer token stored in GCP instance metadata. Retrieved by rc.local at boot to configure Nginx auth."
  type        = string
  sensitive   = true
}

variable "workspace_ip" {
  description = "External IP of the workspace VM. The firewall rule allows port 9443 from this address only."
  type        = string
}
