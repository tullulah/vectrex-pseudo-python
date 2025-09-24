def main():
    # Alterna intensidad cada frame usando un contador global implícito
    if (counter & 1) == 0:
        FRAME_BEGIN(0x5F)
    else:
        FRAME_BEGIN(0x30)
    VECTREX_DBG_STATIC_VL()
    counter = counter + 1
    return 0
