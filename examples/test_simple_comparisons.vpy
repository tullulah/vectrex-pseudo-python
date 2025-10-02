# Test simple de comparaciones corregidas

var counter = 0

def main():
    let dummy = 0

def loop():
    counter = counter + 1
    
    if counter < 30:
        DRAW_LINE(-50, -50, 50, -50, 80)
    elif counter < 60:
        DRAW_LINE(-50, 50, 50, 50, 80)
    else:
        DRAW_LINE(-50, -50, -50, 50, 80)
        
    if counter > 90:
        counter = 0