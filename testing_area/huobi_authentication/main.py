import requests
import hashlib
import hmac
import base64
import urllib.parse
from datetime import datetime
import pytz

API_KEY = 'a54758d1-e326372e-409c8cd7-dab4c45e6f'
SECRET_KEY = 'b9a0467a-ea5426e1-3178806d-68b1b'

def create_signature(secret_key, method, endpoint, params):
    params['SignatureMethod'] = 'HmacSHA256'
    params['SignatureVersion'] = '2'
    params['AccessKeyId'] = API_KEY
    params['Timestamp'] = datetime.utcnow().replace(tzinfo=pytz.utc).strftime("%Y-%m-%dT%H:%M:%S")

    sorted_params = sorted(params.items())
    print("sorted params: ", sorted_params)
    encoded_params = urllib.parse.urlencode(sorted_params)
    
    payload = f"{method}\napi.huobi.pro\n{endpoint}\n{encoded_params}"
    hashed_payload = hmac.new(secret_key.encode(), payload.encode(), hashlib.sha256).digest()
    signature = base64.b64encode(hashed_payload).decode()
    return signature

def send_request(api_key, secret_key):
    method = 'GET'
    endpoint = '/v1/account/accounts'
    params = {}

    signature = create_signature(secret_key, method, endpoint, params)
    print("Signature is ", signature)
    params['Signature'] = signature

    request_url = f"https://api.huobi.pro{endpoint}?{urllib.parse.urlencode(params)}"
    print(f"Request URL: {request_url}")

    response = requests.get(request_url)
    return response.json()

response = send_request(API_KEY, SECRET_KEY)
print(response)
