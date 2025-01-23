terraform {
  backend "s3" {
    profile = "personal"
    bucket  = "mhnap-test-tf"
    key     = "terraform.tfstate"
  }
}

provider "aws" {
  profile = local.aws_profile
}
