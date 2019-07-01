# Running the Website Locally

## Setting Up and Updating

On Debian the following is required:
~~~
apt install ruby-dev libz-dev
~~~

Install Ruby and RubyGems; then install `bundler` and dependencies:
~~~
gem install bundler && bundle install --binstubs
~~~

If https is desired, a private key and cert will be necessary (note that the cert will need to set `-subj /CN=ssl_cert_common_name`):
~~~
openssl genrsa -out website-DEV.key 4096
openssl req -new -x509 -sha256 -key website-DEV.key -out website-DEV.cert -days 9999 -subj /O=website-DEV
~~~

To update all Ruby deps:
~~~
bundle update github-pages
~~~

## Development

To serve the website:
~~~
./bin/jekyll serve -I --watch --host=0.0.0.0
~~~

To serve over https:
~~~
./bin/jekyll serve -I --watch --host=0.0.0.0 --ssl-key website-DEV.key --ssl-cert website-DEV.cert
~~~

# Development Notes

* Color scheme available [at this link](https://color.adobe.com/create/color-wheel/?base=2&rule=Analogous&selected=4&mode=rgb&rgbvalues=0.9098039215686274,0.10980392156862737,0.31264715428561407,0.7761000596538679,0.4489089169755971,0.783921568627451,0.4539901477832555,0.23014778325123153,0.64,0.06378313934435885,0.0001957494578129382,0.55,0.22745098039215686,0.41568627450980394,0.8470588235294118&swatchOrder=0,1,2,3,4&name=My%20Color%20Theme).
* I've spent way too much time trying to bring this to a 100/100 on Lighthouse. Service worker HTTPS keeps complaining and the CSS really isn't small enough for me to bother deferring it (and besides, the HTML spec isn't there yet: *why* there's no `async` or `defer` for a `<link rel="stylesheet">` is beyond me).
* Favicons are treacherous and terrible. Fortunately people like Philippe Bernard exist: he has a guest post over at CSS-Tricks [detailing just how easy it is to get it all wrong](https://css-tricks.com/favicon-quiz/) and he's made [RealFaviconGenerator](http://realfavicongenerator.net/), a tool that eases the pain of it all.
