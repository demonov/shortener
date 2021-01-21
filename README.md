# shortener


## Modules 
mod db - interface for external storage  
mod mocked_db - mocked storage implementation that uses temp folder as database

## API
###GET https://hostname/add/url
- adds a new url to the shortener  

sample request:
```
GET http://127.0.0.1:8080/add/http://www.google.com/
```
sample response:
```
{"short":"2iAwtS6u","long":"http://www.google.com/"}
```


###GET https://hostname/get/key  
- resolves a key into saved url 
  
sample request:
```
http://127.0.0.1:8080/get/2iAwtS6u
```
sample response:
```
{"short":"2iAwtS6u","long":"http://www.google.com/"}
```
