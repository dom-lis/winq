import json

def state(text=[], fg=[], bg=[], style=[]):
    return {'text': text, 'fg': fg, 'bg': bg, 'style': style}

while True:
    s = input()
    print(json.dumps({'State': state(text=[s])}))
