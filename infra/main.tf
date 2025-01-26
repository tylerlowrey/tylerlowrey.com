terraform {
  backend "gcs" {
    bucket = "tl-personal-website-terraform"
    prefix = "personal-website/tofu/state"
  }
}

provider "google" {
  project = "tl-personal-website"
  region = "us-central1"
}
