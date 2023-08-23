<?php

namespace App\Commands;

enum CommandResponseEnum
{
    case SUCCESS;
    case FAIL;
    case INTERNAL;
}
