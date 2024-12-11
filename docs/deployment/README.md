# Запуск проекта на сервере

## Важно!
### На данный момент в рамках запуска через docker не предусмотрена возможность для получения доступа к локальным файлам ПК
### поэтому в рамках отправляемых сообщений в **kafka** использовать только **link_tupe == network**

---

## 1. Используемые зависимости.

* Docker
* Docker-compose

---

## 2. Загрузка моделей.

Необходимый архив с моделями можно установить по [ссылке](https://disk.yandex.ru/d/10OegiujsUYQ1A).\
Расположение моделей на копьютере не имеет значения.

---

## 3. Описание переменных окружения.

В директории **deployment/docker-compose** находится файл **.env** он содержит значения по умолчанию,\
для минимального наполнения и запуска проекта достаточно ввести корректный путь к папке с моделями:

``` .env
# Example of environment variables
KAFKA_HOST = "kafka-dev"
KAFKA_PORT = 9092
KAFKA_TOPIC_INPUT = "pipeline"
KAFKA_TOPIC_OUTPUT = "pipeline_output"

MODELS_PATH = "{локальный путь к директории 'models' (ex: /home/test_user/models/)}"

FACIAL_DETECTOR_MODEL_PATH = "antelopev2/detection/model.onnx"
FACIAL_DETECTOR_MODEL_NAME = "detector"

FACIAL_RECOGNIZER_MODEL_PATH = "antelopev2/recognition/model.onnx"
FACIAL_RECOGNIZER_MODEL_NAME = "recognizer"

TEXTUAL_MODEL_PATH = "clip/text/model.onnx"
TEXTUAL_MODEL_NAME = "sentence-transformers/clip-ViT-B-32-multilingual-v1"

VISUAL_MODEL_PATH = "clip/image/model.onnx"
VISUAL_MODEL_NAME = "visual"

REPLICATES_QTY = 5

```

---

## 4. Запуск проекта.
В директории **backend** необходимо выполнить следующею команду:
``` zsh
# Если пользователь не добавлен в группу Docker
sudo docker compose -f kafka-dc.yml up --build

# Иначе
docker compose -f kafka-dc.yml up --build
```
---

## 5. Создание топиков Kafka для пайплайна (если проект запускается впервые).
``` zsh
docker exec kafka sh /bin/kafka-topics --create --topic pipelines --bootstrap-server localhost:9092
docker exec kafka sh /bin/kafka-topics --create --topic pipelines_output --bootstrap-server localhost:9092
```
или запустите скрипт **deployment/kafka/create_topics_default.sh**
``` zsh
sh deployment/kafka/create_topics_default.sh
```

---
---


# Использование

1) Для мониторинга входящих **messages, purtiotions, topics**, перейдите по ссылке **http://0.0.0.0:8080/**.

2) В качестве альтернативы готовому клиенту приложения можно использовать приложение из первого пункта. \
Предварительно создайте топик с названием **pipelines** и отправьте следующее сообщение:
- key = [укажите расширение вашего файла];
- value =
``` json
{
  "link": "[путь к файлу с учётом указанного расширения файла в ключе]",
  "link_type": "[file или network]"
}
```
