test = 0

def _draw():
  global test

  test = test + 1
  clear_screen(0.0, 1.0, 0.0)
  draw_sprite(0, test, 0)
  draw_rect(0.0, 0.0, 1.0, 0.0, 0.0, 8.0, 8.0)