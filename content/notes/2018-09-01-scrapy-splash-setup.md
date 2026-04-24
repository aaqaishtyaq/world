+++
title = "How to Set Up the Scrapy Splash Plugin"
date = "2018-09-01"
+++

Scrapy is great for scraping static web pages with Python, but when it comes to
dynamic pages it can only do so much. That's where ```Selenium``` usually comes
in, but as good as Selenium is, Scrapy still beats it in terms of speed.

The web these days is full of dynamic JS-based pages and AJAX. For exactly that
scenario, the folks over at [scrapy-plugins][0] created ```scrapy-splash```.
Scrapy-Splash is a plugin that connects Scrapy with Splash (a lightweight,
scriptable browser as a service with an HTTP API). In a nutshell, Splash takes
the response received from the server and renders it. Then it returns a
```render.html``` response which is static and can be scraped easily.

## 0 - Setting up the machine

A. Before we begin you need to install ```Docker``` first, You can follow the [official instruction][1] as per your Operating System.

B. After installing Docker, navigate to your project folder, activate
```virtualenv``` and install the scrapy-splash plugin

```bash
pip3 install scrapy-splash
```

C. Pull the Splash Docker Image and run it

```bash
docker pull scrapinghub/splash
docker run -p 8050:8050 scrapinghub/splash
```

## 1 - Configuration

A. Add the Splash server address to ```settings.py``` of your Scrapy project like this:

```python
SPLASH_URL = 'http://localhost:8050'
```
If you are running docker on your local machine then you can simply use ```http://localhost:<port>``` , but if you are running it on a remote machine you need to specify it's I.P. Address like this ```http://192.168.59.103:<port>```


B. Enable the Splash middleware by adding it to ```DOWNLOADER_MIDDLEWARES``` in your ```settings.py``` file and changing HttpCompressionMiddleware priority:

```python
DOWNLOADER_MIDDLEWARES = {
    'scrapy_splash.SplashCookiesMiddleware': 723,
    'scrapy_splash.SplashMiddleware': 725,
    'scrapy.downloadermiddlewares.httpcompression.HttpCompressionMiddleware': 810,
}
```


C. Enable SplashDeduplicateArgsMiddleware by adding it to SPIDER_MIDDLEWARES in your settings.py:

```python
SPIDER_MIDDLEWARES = {
    'scrapy_splash.SplashDeduplicateArgsMiddleware': 100,
}
```


D. Set a custom DUPEFILTER_CLASS:

```python
DUPEFILTER_CLASS = 'scrapy_splash.SplashAwareDupeFilter'
```

## 2 - Scraping with Splash

Before you use ```scrapy-splash``` you need to import it in your spider.
You can do that by adding this line:

```python
from scrapy_splash import SplashRequest
```

from now on insted of using ```scrapy.Request``` you can simply use ```SplashRequest``` to get response from ```Splash``` insted of directly from ther server.

## Bonus - Using Scrapy-Splash in Shell

It's all well and good but actual spider buiding does not happens in ```vim``` or ```sublime```, it takes place in ```shell```.

**So how to use Splash in the shell?**

Good Question.

Insted of invoking shell with:

```bash
scrapy shell
>>> fetch(http://domain.com/page-with-javascript.html)
```
or with this:

```bash
scrapy shell http://domain.com/page-with-javascript.html
```

**You invoke shell with this**:

```bash
scrapy shell 'http://localhost:8050/render.html?url=http://domain.com/page-with-javascript.html&timeout=10&wait=0.5'
```

Let me explain

* ```localhost:port``` is where your splash service is running
* ```url``` is url you want to crawl
* ```render.html``` is one of the possible http api endpoints, returns redered html page in this case
* ```timeout``` time in seconds for timeout
* ```wait``` time in seconds to wait for javascript to execute before reading/saving the html.


If I’ve missed something, made a horrible mistake of if you have any questions regarding this article then feel free to ping me on Twitter. I’m
[@aaqaishtyaq](https://twitter.com/aaqaishtyaq).

[0]: https://github.com/scrapy-plugins
[1]: https://docs.docker.com/install/
