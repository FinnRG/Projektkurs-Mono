FROM node:lts-alpine as builder

WORKDIR /app

COPY ./package.json ./
RUN npm install

COPY . .
RUN npm run build

FROM nginx:stable-alpine

RUN rm /etc/nginx/conf.d/default.conf  # <= This line solved my issue
COPY nginx/nginx.conf /etc/nginx/conf.d
EXPOSE 80

COPY --from=builder /app/dist /usr/share/nginx/html/frontend