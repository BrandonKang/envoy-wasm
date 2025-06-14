# echo-api/Dockerfile 
FROM python:3.9-slim

WORKDIR /app

COPY app.py /app/app.py

RUN pip install flask

EXPOSE 8081

CMD ["python", "app.py"]

