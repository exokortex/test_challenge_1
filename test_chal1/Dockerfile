FROM registry.losfuzzys.net/fuzzyland/k8ctf2/challenges/pwnbase/pwntest
WORKDIR /

COPY ./exploit.py /usr/src/app/
ENV EXPLOIT "python /usr/src/app/exploit.py {TARGET0_PROTOCOL}://{TARGET0_HOST}:{TARGET0_PORT}"
