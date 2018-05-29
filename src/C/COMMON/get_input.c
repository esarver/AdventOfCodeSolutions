#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <curl/curl.h>

/***************************************************************************
 *                                  _   _ ____  _
 *  Project                     ___| | | |  _ \| |
 *                             / __| | | | |_) | |
 *                            | (__| |_| |  _ <| |___
 *                             \___|\___/|_| \_\_____|
 *
 * Copyright (C) 1998 - 2015, Daniel Stenberg, <daniel@haxx.se>, et al.
 *
 * This software is licensed as described in the file COPYING, which
 * you should have received as part of this distribution. The terms
 * are also available at https://curl.haxx.se/docs/copyright.html.
 *
 * You may opt to use, copy, modify, merge, publish, distribute and/or sell
 * copies of the Software, and permit persons to whom the Software is
 * furnished to do so, under the terms of the COPYING file.
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY OF ANY
 * KIND, either express or implied.
 *
 ***************************************************************************/ 
/* <DESC>
 * Shows how the write callback function can be used to download data into a
 * chunk of memory instead of storing it in a file.
 * </DESC>
 */ 

/*
Example URL for input: http://adventofcode.com/2015/day/1/input
*/

struct MemoryStruct {
	char * memory;
	size_t size;
}

static size_t WriteMemoryCallback(void *contents, size_t size, size_t nmemb, void *userp){
	size_t realsize = size * nmemb;
	struct MemoryStruct * mem = (struct MemoryStruct *) userp;

	mem->memory = realloc(mem->memory, mem->size + realsize + 1);
	if(mem->memory == NULL) {
		/* out of memory*/
		printf("Not enough memory (realloc returned NULL\n");
		return 0;
	}

	memcpy(&(mem->memory[mem->size]), contents, realsize);
	mem->size += realsize;
	mem->memory[mem->size] = 0;

	return realsize;
}


