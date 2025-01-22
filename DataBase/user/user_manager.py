from pymongo import MongoClient
from bson.objectid import ObjectId
import os

def get_database():
    mongo_uri = os.getenv("MONGO_URI", "mongodb://localhost:27017/")
    client = MongoClient(mongo_uri)
    db = client['user_database']
    return db

def register_user(data):
    db = get_database()
    users = db['users']

    if users.find_one({"username": data['username']}):
        return {"success": False, "message": "Username already exists"}

    try:
        user_data = {
            "username": data['username'],
            "password": data['password'],
            "email": data['email'],
            "data": {},
            "chats": {}
        }
        users.insert_one(user_data)
        return {"success": True, "message": "User registered successfully"}
    except Exception as e:
        return {"success": False, "message": f"An error occurred: {str(e)}"}

def login_user(data):
    db = get_database()
    users = db['users']

    user = users.find_one({"username": data['username']})
    if user and user['password'] == data['password']:
        return {"success": True, "message": "Login successful"}
    else:
        return {"success": False, "message": "Invalid username or password"}

def update_data(data):
    db = get_database()
    users = db['users']

    user = users.find_one({"username": data['username']})
    if not user:
        return {"success": False, "message": "User not found"}

    update_options = {
        'username': lambda value: {"$set": {"username": value}},
        'password': lambda value: {"$set": {"password": value}},
        'email': lambda value: {"$set": {"email": value}},
        'data': lambda value: {"$set": {"data": value}},
        'chats': lambda value: {"$set": {"chats": value}}
    }

    option = data.get('option')
    new_value = data.get('data')

    if option not in update_options:
        return {"success": False, "message": "Invalid option"}

    if not new_value:
        return {"success": False, "message": f"New {option} is required"}

    if option == 'username' and users.find_one({"username": new_value}):
        return {"success": False, "message": "Username already exists"}

    update_query = update_options[option](new_value)
    users.update_one({"username": data['username']}, update_query)

    return {"success": True, "message": f"{option.capitalize()} updated successfully"}

