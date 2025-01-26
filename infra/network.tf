resource  "google_compute_network" "personal_website_network" {
   name = "personal-website-network"
   project = "tl-personal-website"
   auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "personal_website_subnetwork_main" {
   name          = "personal-website-subnetwork-main"
   ip_cidr_range = "10.10.1.0/24"
   network       = google_compute_network.personal_website_network.name
   region        = "us-central1"
}

resource "google_compute_address" "personal_website_server_address" {
  name = "personal-website-server-address"
  address_type = "EXTERNAL"
}

resource "google_compute_firewall" "allow_http_https" {
  name = "personal-website-firewall-allow-http-https"
  network = google_compute_network.personal_website_network.name
  allow {
    ports = ["80", "443"]
    protocol = "tcp"
  }
  source_ranges = ["0.0.0.0/0"]
  target_tags = ["public"]
}

resource "google_compute_firewall" "allow_ssh" {
  name = "personal-website-firewall-allow-ssh"
  network = google_compute_network.personal_website_network.name
  allow {
    ports = ["22"]
    protocol = "tcp"
  }
  source_ranges = ["0.0.0.0/0"]
  target_tags = ["public"]
}
