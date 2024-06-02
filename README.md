## Rust Bucket

Rust Bucket is a simple file upload and management service built with Rust and Rocket.

### Features

- **File Upload**: Upload files to a designated bucket directory.
- **File Download**: Download files from the bucket.
- **List Files**: Retrieve a list of file names stored in the bucket.
- **File Deletion**: Delete files from the bucket.

### Getting Started

To run Rust Bucket, follow these steps:

#### Prerequisites

- Docker installed on your system.

#### Instructions

1. Clone the repository:

    ```bash
    git clone <repository_url>
    ```

2. Navigate to the project directory:

    ```bash
    cd rust-bucket
    ```

3. Build the Docker image:

    ```bash
    docker build -t rust-bucket .
    ```

4. Run the Docker container:

    ```bash
    docker run -d -p 8000:8000 rust-bucket
    ```

### API Endpoints

- **Upload File**: `POST /bucket/upload_file`
  - Upload a file to the bucket directory.
- **Download File**: `GET /bucket/download_file/<file_name>`
  - Download a file from the bucket.
- **List Files**: `GET /bucket/get_file_names`
  - Retrieve a list of file names stored in the bucket.
- **Delete Files**: `DELETE /bucket/delete_files?file_names=<comma_separated_file_names>`
  - Delete files from the bucket.

### API Usage

#### Upload File

```
POST /bucket/upload_file
Content-Type: multipart/form-data

[file] # File to be uploaded
```

#### Download File

```
GET /bucket/download_file/<file_name>
```

#### List Files

```
GET /bucket/get_file_names
```

#### Delete Files

```
DELETE /bucket/delete_files?file_names=<comma_separated_file_names>
```