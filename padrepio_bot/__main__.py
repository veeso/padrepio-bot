#!/usr/bin/python3

"""
            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004
 Copyright (C) 2023 Christian "veeso" Visintin
 Everyone is permitted to copy and distribute verbatim or modified
 copies of this license document, and changing it is allowed as long
 as the name is changed.
            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
  0. You just DO WHAT THE FUCK YOU WANT TO.
"""

from os import environ
import logging
from logging import info, debug, error
import requests
from telegram.ext.updater import Updater
from telegram.update import Update
from telegram.ext.callbackcontext import CallbackContext
from telegram.ext.commandhandler import CommandHandler

TELEGRAM_API_KEY = environ["TELEGRAM_API_KEY"]
LOG_LEVEL = environ.get("LOG_LEVEL", "info")


def main() -> None:
    logging.basicConfig(
        format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
        level=get_loglevel(LOG_LEVEL),
    )

    updater = Updater(TELEGRAM_API_KEY, use_context=True)
    info(f"initialized telegram updater {TELEGRAM_API_KEY}")
    updater.dispatcher.add_handler(CommandHandler("start", start))
    updater.dispatcher.add_handler(CommandHandler("padrepio", padrepio))
    updater.dispatcher.add_handler(CommandHandler("help", help))
    info("starting telegram bot")
    # Start the Bot
    info("starting bot")
    updater.start_polling()
    updater.idle()
    exit(0)


def send_request(text: str) -> str:
    headers = {
        "Accept": "*/*",
        "Accept-Language": "en-GB,en;q=0.6",
        "Cache-Control": "no-cache",
        "Connection": "keep-alive",
        # Already added when you pass json=
        # 'Content-Type': 'application/json',
        "Origin": "https://www.prega.org",
        "Pragma": "no-cache",
        "Referer": "https://www.prega.org/",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "cross-site",
        "Sec-GPC": "1",
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36",
    }
    data = f"{text}AI:"

    response = requests.post(
        "https://pregaproxy.azurewebsites.net/api/proxy/PadrePio",
        headers=headers,
        json=data,
    )
    out_data = response.json()
    return out_data["choices"][0]["text"].strip()


def start(update: Update, _: CallbackContext):
    update.message.reply_text(
        "Buongiorno, benvenuto. Io sto pregando e prestando servizio ai fratelli e sorelle bisognosi. La pace di Gesù sia con te. Chiedimi qualsiasi cosa con /padrepio <domanda>"
    )


def padrepio(update: Update, context: CallbackContext):
    text: str = (
        update.message.text.replace("/padrepio", "")
        .replace(context.bot.name, "")
        .strip()
    )
    debug(f"text: {text}")
    if len(text) == 0:
        context.bot.send_message(
            chat_id=update.message.chat_id, text="Ti ascolto figliolo"
        )
        return
    try:
        debug("sending request to padre pio")
        answer = send_request(text)
    except Exception as err:
        error(f"failed to get tts speech: {err}")
        return reply_err(update, f"Non riesco a contattare Padre Pio: {err}")
    debug("got an answer from padre pio")
    context.bot.send_message(chat_id=update.message.chat_id, text=answer)


def reply_err(update: Update, text: str):
    update.message.reply_text(text)


def help(update: Update, _: CallbackContext):
    update.message.reply_text(
        """/padrepio <testo> - chiedi consiglio a padre pio 
/help - mostra questo messaggio"""
    )


def get_loglevel(level: str) -> int:
    try:
        return {
            "info": logging.INFO,
            "error": logging.ERROR,
            "debug": logging.DEBUG,
            "warn": logging.WARN,
        }.get(level)
    except Exception:
        return logging.ERROR


if __name__ == "__main__":
    main()
