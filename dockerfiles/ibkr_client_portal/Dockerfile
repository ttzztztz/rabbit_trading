FROM amazoncorretto:21-al2023

RUN yum install -y unzip

RUN mkdir -p /opt/ibkr
WORKDIR /opt/ibkr

ADD https://download2.interactivebrokers.com/portal/clientportal.gw.zip /opt/ibkr/clientportal.gw.zip
RUN unzip /opt/ibkr/clientportal.gw.zip

ENTRYPOINT [ "/opt/ibkr/bin/run.sh", "root/conf.yaml" ]
