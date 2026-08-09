resource "google_cloud_run_v2_service" "nawawan_prod_service" {
  provider            = google
  project             = var.project
  location            = var.region
  deletion_protection = false
  name                = "nawawan-prod-service"
  ingress             = "INGRESS_TRAFFIC_ALL"

  scaling {
    max_instance_count = 2
    min_instance_count = 0
  }

  lifecycle {
    ignore_changes = [template[0].containers[0].image]
  }

  template {
    service_account = google_service_account.nawawan_prod_reader.email

    containers {
      image = "asia-northeast1-docker.pkg.dev/nawawan/nawawan-prod-repository/nawawan-playground:v1.1.0"

      env {
        name  = "ENV"
        value = "prod"
      }
      env {
        name = "DATABASE_URL"
        value_source {
          secret_key_ref {
            secret  = google_secret_manager_secret.prod_database_url.secret_id
            version = "latest"
          }
        }
      }
      env {
        name = "CLOUDFLARE_ACCESS_KEY_ID"
        value_source {
          secret_key_ref {
            secret  = google_secret_manager_secret.prod_cloudflare_access_key_id.secret_id
            version = "latest"
          }
        }
      }
      env {
        name = "CLOUDFLARE_SECRET_ACCESS_KEY"
        value_source {
          secret_key_ref {
            secret  = google_secret_manager_secret.prod_cloudflare_secret_access_key.secret_id
            version = "latest"
          }
        }
      }
      env {
        name = "CLOUDFLARE_ACCOUNT_ID"
        value_source {
          secret_key_ref {
            secret  = google_secret_manager_secret.prod_cloudflare_account_id.secret_id
            version = "latest"
          }
        }
      }    
      env {
        name = "CF_ACCESS_TEAM_DOMAIN"
        value = "nawawan.cloudflareaccess.com"
      }
      env {
        name = "CF_ACCESS_AUD"
        value = "756ccca02f1b6db0c85afe0ff0edf345ee5a8bed3b56d65f6b6178e9d2bc3b5a"
      }
      env {
        name = "BLOG_R2_BUCKET"
        value = "blog-assets-prod"
      }
      env {
        name  = "PAGE_HOST"
        value = "https://image.nawawan.com"
      }
    }
  }
}
