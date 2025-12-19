META TITLE = "Simple Test"

x = 10  # Global

def main():
    y = 20  # Local (primera asignación en función, no global)

def loop():
    WAIT_RECAL()
    z = x  # Local z (primera asignación aquí)
