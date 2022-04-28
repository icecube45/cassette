#!/usr/bin/env python

import asyncio
import random
import websockets
import json


width = 30
height = 10
num_pixels = width*height
pixels = [None]*(num_pixels)

def wheel(pos):
    # Input a value 0 to 255 to get a color value.
    # The colours are a transition r - g - b - back to r.
    if pos < 0 or pos > 255:
        r = g = b = 0
    elif pos < 85:
        r = int(pos * 3)
        g = int(255 - pos * 3)
        b = 0
    elif pos < 170:
        pos -= 85
        r = int(255 - pos * 3)
        g = 0
        b = int(pos * 3)
    else:
        pos -= 170
        r = 0
        g = int(pos * 3)
        b = int(255 - pos * 3)
    return (r, g, b)


def strip_to_matrix():
    for i in range(9):
        j=i+1
        pixels[j*30:j*30+30] = pixels[:30]


async def consumer(message):
    # while True:
    #     val = await queue.get()
    #     print('{} get a val: {}'.format(id, val))
    #     await asyncio.sleep(1)
    #     queue.task_done()   # indicate complete task
    pass


step = 0


def rgb_to_pixel_entry(rgb, patched):
    print(patched)
    return {
        "r": rgb[0],
        "g": rgb[1],
        "b": rgb[2],
        "patched": patched
    }



async def rainbow(matrix=False):
    global step
    num_pixels_override=0
    if(matrix):
        num_pixels_override = num_pixels
    else:
        num_pixels_override = width

    for i in range(num_pixels):
        pixel_index = (i * 256 // num_pixels_override) + step
        patch = True
        # if(i%3 == 0):
        #     patch = False
        pixels[i] = rgb_to_pixel_entry(wheel(pixel_index & 255), patch)
    # await asyncio.sleep(0.005)
    step += 1
    if(step == 256):
        step = 0

    if(matrix):
        strip_to_matrix()
   
async def producer():
    await rainbow()
    # for pixel in pixels:
    #     #get rand between 0 and 1
    #     if(random.random() > 0.5):
    #         pixel['patched'] = False

    return json.dumps(pixels)







async def handler(websocket, path):
    while True:
        listener_task = asyncio.ensure_future(websocket.recv())
        producer_task = asyncio.ensure_future(producer())
        done, pending = await asyncio.wait(
            [listener_task, producer_task],
            return_when=asyncio.FIRST_COMPLETED)

        if listener_task in done:
            message = listener_task.result()
            await consumer(message)
        else:
            listener_task.cancel()

        if producer_task in done:
            message = producer_task.result()
            # print(message)
            await websocket.send(message)
        else:
            producer_task.cancel()


start_server = websockets.serve(handler, 'localhost', 3000)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()