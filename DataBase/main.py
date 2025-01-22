import zmq
from db import db_manager
from user import user_manager

def handle_request(request):
    action = request.get("action")
    if action == "register":
        return user_manager.register_user(request)
    elif action == "login":
        return user_manager.login_user(request)
    elif action == "update_data":
        return user_manager.update_data(request)
    elif action == "user_list":
        return
    else:
        return {"success": False, "message": "Unknown action"}

if __name__ == "__main__":
    context = zmq.Context()
    socket = context.socket(zmq.REP)
    socket.bind("tcp://127.0.0.1:25378")

    print("Python DB handler is running...")
    while True:
        message = socket.recv_json()
        response = handle_request(message)
        socket.send_json(response)