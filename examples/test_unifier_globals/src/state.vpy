# State module - manages global counter

counter = 0

def init_state():
    counter = 0

def increment():
    counter = counter + 1

def get_counter():
    return counter
