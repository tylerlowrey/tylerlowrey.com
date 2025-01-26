resource "google_compute_instance" "personal_website_server" {
  name = "personal-website-server"
  project = "tl-personal-website"
  machine_type = "f1-micro"
  zone = "us-central1-c"

  tags = ["personal-website-server", "public"]
  
  boot_disk {
    initialize_params {
      image = "debian-cloud/debian-12"
      size = "15"
      labels = {
        application = "personal-website"
      }
    }
  }

  network_interface {
    subnetwork = google_compute_subnetwork.personal_website_subnetwork_main.name
    access_config {
      nat_ip = google_compute_address.personal_website_server_address.address
    }
  }
}
