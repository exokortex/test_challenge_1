#!/usr/bin/env python3

from flask import Flask, request

app = Flask(__name__, static_folder='static', static_url_path='')


@app.route('/')
def root():
    ret_str = '''
    Up & Running (Server2)! <br> <br>
    Welcome to our crawler-free website :)
    '''

    return ret_str


@app.route('/n0_cr4wl3rs_plzZz', methods=['GET'])
def no_crawlers_pls():
    user_agent = request.user_agent.string

    if user_agent != "H4xx3r":
        ret_str = '''
        Still? <br>
        No crawler, please! 
        '''
    else:
        ret_str = '''
        You are no crawler, are you? <br>
        Here is your flag :)<br> <br> %s 
        ''' % 'LosCTF{r0b0ts.txt_t0_th3_r3scu3?}'

    return ret_str


if __name__ == "__main__":
    app.run()


