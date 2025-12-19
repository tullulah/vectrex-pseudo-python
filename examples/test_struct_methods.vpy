META TITLE = "STRUCT METHODS TEST"

struct Point:
    x: int
    y: int
    
    def move(dx, dy):
        self.x = self.x + dx
        self.y = self.y + dy
    
    def distance_from_origin():
        return abs(self.x) + abs(self.y)

def main():
    pass

def loop():
    p = Point()
    p.x = 10
    p.y = 20
    p.move(5, -10)
    dist = p.distance_from_origin()
