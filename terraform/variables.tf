variable "bucket_name" {
  description = "Value of the bucket name"
  default     = "mhnap-test-bucket"
  type        = string
}

variable "bucket_tag_key" {
  default = "key"
}

variable "bucket_tag_value" {
  default = "value"
}
