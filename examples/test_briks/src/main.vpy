META TITLE = "BRIKS"
META COPYRIGHT = "g 2025"
META MUSIC = 1

var paddle_x = 0
var ball_x = 0
var ball_y = -80
var ball_dx = 2
var ball_dy = 2

def main():
    PLAY_MUSIC("music")

def loop():
    MUSIC_UPDATE()
    SET_ORIGIN()
    SET_INTENSITY(127)
    
    let joy = J1_X()
    if joy < 0:
        paddle_x = paddle_x - 3
    if joy > 0:
        paddle_x = paddle_x + 3
    if paddle_x < -100:
        paddle_x = -100
    if paddle_x > 100:
        paddle_x = 100
    
    ball_x = ball_x + ball_dx
    ball_y = ball_y + ball_dy
    
    if ball_x < -120:
        ball_dx = 0 - ball_dx
    if ball_x > 120:
        ball_dx = 0 - ball_dx
    if ball_y > 120:
        ball_dy = 0 - ball_dy
    if ball_y < -120:
        ball_x = 0
        ball_y = -80
    
    DRAW_VECTOR("paddle", paddle_x, -100)
    DRAW_VECTOR("ball", ball_x, ball_y)
    DRAW_VECTOR("brick", -90, 80)
    DRAW_VECTOR("brick", -20, 80)
    DRAW_VECTOR("brick", 20, 80)
    DRAW_VECTOR("brick", 90, 80)
