![Oh hello](img/isay.png)

# Archibald 

To be a butler, is to be able to maintain an even-temper, at all times. One must have exceptional personal hygiene and look sharp and professional, always. Even when under stress or scrutiny, a butler must remain calm and lead their team through the difficulties.

Archibald is my attempt at learning Rust and writing a HTTP 1.1 web server. 

## Architecture

![](img/architecture.png)

We shall be adopting the KISS approach to building things. I mean how hard is parsing modern web languages and content? 

## How Does HTTP Actually Work?

For those who aren't aware, Hypertext Transfer Protocol (HTTP) is a [layer 7](https://en.wikipedia.org/wiki/OSI_model) (application) protocol. The whole thing works by requests and responses, the latter being accepted by a server, which provides the answer. HTTP is stateless and this makes it more fun in a way. 

It all looks like this:

```
daniel@sexy ~/Code/Archibald -> main -> nc -vv nsa.gov 80
Notice: Real hostname for nsa.gov [23.63.141.16] is a23-63-141-16.deploy.static.akamaitechnologies.com
nsa.gov [23.63.141.16] 80 (http) open
GET / HTTP/1.1
```
That's connecting to the server, on port 80 and asking for the index. It responds:

```
HTTP/1.0 400 Bad Request
Server: AkamaiGHost
Mime-Version: 1.0
Content-Type: text/html
Content-Length: 209
Expires: Tue, 03 May 2022 13:36:08 GMT
Date: Tue, 03 May 2022 13:36:08 GMT
Connection: close

<HTML><HEAD>
<TITLE>Invalid URL</TITLE>
</HEAD><BODY>
<H1>Invalid URL</H1>
The requested URL "&#91;no&#32;URL&#93;", is invalid.<p>
Reference&#32;&#35;9&#46;1ef01602&#46;1651584968&#46;16093878
</BODY></HTML>
Total received bytes: 419
Total sent bytes: 16
```
### Understanding HTTP Messages

There are two types of HTTP messages, requests and responses, each with its own format.

**Requests**

GET (Method)
/ (Path)
HTTP /1.1 (Protocol Version)

**Responses**

HTTP/1.0 (Protocol Version)
400 (Status Code)
Bad Request (Status Message)

When building a server, it's important to know how this all works. 

## Threat Modeling

![Oh hello](img/architecture2.png)

I'm sure no-one will dare to attack this, but just in case, we shall be performing a threat modeling exercise so we understand the threats and code appropriate countermeasures. 

![Oh hello](img/threats.png)

## Disclaimer

This will not be production ready, it might eat your children and cause you to like Lotus Notes. I'm not professing to be an expert in Rust and therefore treat this as pretty dodgy. 