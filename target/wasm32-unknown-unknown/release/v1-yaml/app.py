from flask import Flask, jsonify
app = Flask(__name__)

@app.route("/")
def index():
    return jsonify({
        "status": "ok",
        "message": "Hi KubeCon Japan! This is a response from the API Server"
    })

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8081)

