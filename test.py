#!/usr/bin/env python3.6

import socket
import json

json_1 = {"user_key":"alkjv092itg92hf","challenge_key":"win-host-name","flag":"flag-chckyoself"}

json_2 = {"user_key":"28fghavnoa9250","challenge_key":"win-username"}

json_3 = '{"user_key":"cryptik","challenge_key":"win-txtdoc","flag":"flag(isthisreallife)"}'

data_json1 = json.dumps(json_1)#.replace('"','').replace('\\','')
data_json2 = json.dumps(json_2)
data_json3 = json.dumps(json_3)


def main():
    print(data_json1)
    print(data_json2)
    print(data_json3)
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    target = ("127.0.0.1",1234)
    s.connect(target)
    # s.send(data_json1.encode('utf-8'))
    s.send(data_json2.encode('utf-8'))
    # s.send(data_json3.encode('utf-8'))
    s.shutdown(socket.SHUT_WR)
    print(s.recv(2048))


if __name__ == '__main__':
    main()
