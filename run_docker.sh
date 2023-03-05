docker stop web_thai_nlp_x
docker rm web_thai_nlp_x

docker run --name web_thai_nlp_x  -p 8000:8000 -d web_thai_nlp_x:latest