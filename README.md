# HHton Service Development

## Environment Configuration

The service expects an S3 bucket configuration to be available at runtime. When
running with Docker Compose this is provided automatically, but developers
running the application outside of Compose must ensure the `S3_BUCKET`
environment variable is set:

```bash
export S3_BUCKET=hhton-dev
```

Point the value at the correct bucket for your environment (for local
development this is typically the MinIO bucket created by the stack).
