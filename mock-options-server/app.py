from flask import Flask, jsonify, request
import random
from datetime import datetime, timedelta

app = Flask(__name__)

def generate_random_option():
    expiration = (datetime.now() + timedelta(days=random.randint(1, 365))).strftime("%Y-%m-%d")
    strike = round(random.uniform(50, 200), 2)
    option_type = random.choice(["call", "put"])
    
    return {
        "contractID": f"IBM{expiration.replace('-', '')}{'C' if option_type == 'call' else 'P'}{int(strike * 1000):08d}",
        "symbol": "IBM",
        "expiration": expiration,
        "strike": f"{strike:.2f}",
        "type": option_type,
        "last": f"{random.uniform(0, 50):.2f}",
        "mark": f"{random.uniform(0, 100):.2f}",
        "bid": f"{random.uniform(0, 100):.2f}",
        "bid_size": str(random.randint(1, 100)),
        "ask": f"{random.uniform(0, 100):.2f}",
        "ask_size": str(random.randint(1, 100)),
        "volume": str(random.randint(0, 10000)),
        "open_interest": str(random.randint(0, 10000)),
        "date": datetime.now().strftime("%Y-%m-%d"),
        "implied_volatility": f"{random.uniform(0, 5):.5f}",
        "delta": f"{random.uniform(-1, 1):.5f}",
        "gamma": f"{random.uniform(0, 0.1):.5f}",
        "theta": f"{random.uniform(-1, 0):.5f}",
        "vega": f"{random.uniform(0, 0.1):.5f}",
        "rho": f"{random.uniform(-0.1, 0.1):.5f}"
    }

@app.route('/query', methods=['GET'])
def historical_options():
    num_options = random.randint(50, 200)
    options_data = [generate_random_option() for _ in range(num_options)]

    response = {
        "endpoint": "Historical Options",
        "message": "success",
        "data": options_data
    }

    return jsonify(response)

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)
