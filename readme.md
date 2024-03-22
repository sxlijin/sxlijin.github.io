# hi, i'm sam!	

notes on how my personal website is implemented.	

## dev process	

to run incremental builds:	

```bash	
npx nodemon -e md,sh,scss --exec ./build.sh	
```	

to run hot reload:	

```bash	
npx browser-sync start --server _site --files _site --extensions html	
```	

to iterate on Lua filters:	

```bash	
pandoc FILE -t json	
pandoc FILE -t lib/writer.lua	
npx nodemon -e lua --exec pandoc FILE -t lib/writer.lua	
```	

## TODOs	

* Improve blog.md auto-generation (use post titles)	
* Show dates on blog posts	
* Revisit the styling - I think I really liked pandoc's built-in styles	
  * Use `pandoc -D html` to check out the default HTMl template	

## Changelog	

* Dec 2023 - converted to Pandoc from Jekyll	
* Jan 2017 - cleaned it up, spent a lot of time on Lighthouse optimizations	
* Sept 2016 - created this website	

## Development notes	

* Color scheme available [at this link](https://color.adobe.com/create/color-wheel/?base=2&rule=Analogous&selected=4&mode=rgb&rgbvalues=0.9098039215686274,0.10980392156862737,0.31264715428561407,0.7761000596538679,0.4489089169755971,0.783921568627451,0.4539901477832555,0.23014778325123153,0.64,0.06378313934435885,0.0001957494578129382,0.55,0.22745098039215686,0.41568627450980394,0.8470588235294118&swatchOrder=0,1,2,3,4&name=My%20Color%20Theme).	
* Favicons are treacherous and terrible. Fortunately people like Philippe Bernard exist: he has a guest post over at CSS-Tricks [detailing just how easy it is to get it all wrong](https://css-tricks.com/favicon-quiz/) and he's made [RealFaviconGenerator](http://realfavicongenerator.net/), a tool that eases the pain of it all.
