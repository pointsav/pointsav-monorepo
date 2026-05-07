output "yoyo_external_ip" {
  description = "External IP of the Yo-Yo instance. Set SLM_YOYO_ENDPOINT=https://<ip>:9443 in local-doorman.env."
  value       = google_compute_instance.yoyo_tier_b.network_interface[0].access_config[0].nat_ip
}

output "weights_disk_name" {
  description = "Name of the persistent weights disk. Use with gcloud compute scp to upload model files."
  value       = google_compute_disk.weights.name
}

output "instance_name" {
  description = "GCE instance name. Set SLM_YOYO_GCP_INSTANCE to this value in local-doorman.env."
  value       = google_compute_instance.yoyo_tier_b.name
}

output "zone" {
  description = "GCE zone. Set SLM_YOYO_GCP_ZONE to this value in local-doorman.env."
  value       = var.zone
}
