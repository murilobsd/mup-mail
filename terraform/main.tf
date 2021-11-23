terraform {
  required_version = ">= 1.0.10"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "3.63.0"
    }
  }

  /*
   *  Using S3, DynamoDB to state file and global lock
   *
   *  This key will change depending on the application, for now the default will be:
   *  "environment/service/terraform.state"
   */
  backend "s3" {
    bucket         = "mup-terraform-state"
    key            = "dev/mup-user/terraform.tfstate"
    dynamodb_table = "terraform-state-locking"
    encrypt        = true
    region         = "us-east-1"
  }
}

provider "aws" {
  region = var.region
}

/*
 * S3 Terraform State
 */
resource "aws_s3_bucket" "terraform_state" {
  bucket = "terraform-state-kovi-dev"
  acl    = "private"

  versioning {
    enabled = true
  }

  lifecycle {
    prevent_destroy = true
  }

  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "AES256"
      }
    }
  }
}

/*
 * DynamoDB Terraform Global Lock
 * NOTE: Not change attribute name and hash_key **EVER**
 */
resource "aws_dynamodb_table" "terraform_locks" {
  name         = "terraform-state-locking"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "LockID"

  attribute {
    name = "LockID"
    type = "S"
  }
}
