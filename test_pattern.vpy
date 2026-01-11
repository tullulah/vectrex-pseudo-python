META TITLE = "Pattern Test"
META ROM_TOTAL_SIZE = 524288
META ROM_BANK_SIZE = 16384

# These are like pang_multi
const location_x = [40, 40]
const location_y = [110, 79]
const location_names = ["LOC1", "LOC2"]

const level_count = [1, 2]
const level_speed = [1, 2]

joystick1_state = [0,0,0,0,0,0]

const num_locations = 17
current_location = 0
location_glow_intensity = 60

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    val = joystick1_state[0]
    SET_INTENSITY(val + 10)
