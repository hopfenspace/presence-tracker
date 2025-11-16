FROM nginx:latest AS final

COPY ./frontend /usr/share/nginx/html
