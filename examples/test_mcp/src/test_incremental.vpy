META TITLE = "Incremental Draw Test"

def main():
    SET_INTENSITY(127)

def loop():
    # ITERACIÓN 11: Usar DRAW_VECTOR builtin (loop completo en Draw_Sync_List)
    WAIT_RECAL()
    DRAW_VECTOR("triangle")
