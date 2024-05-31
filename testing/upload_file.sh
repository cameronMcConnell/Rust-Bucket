curl -F 'file=@test_files/test1.txt' 127.0.0.1:8000/bucket/upload_file
curl -F 'file=@test_files/test2.txt' 127.0.0.1:8000/bucket/upload_file
curl -F 'file=@test_files/test3.txt' 127.0.0.1:8000/bucket/upload_file
curl -F 'file=@test_files/sanatize_test@#$.txt' 127.0.0.1:8000/bucket/upload_file