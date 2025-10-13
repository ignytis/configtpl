#ifdef _WIN32
#include <stdlib.h>
#endif
#include <stdio.h>

#include "../../../../include/configtpl.h"

#define MAX_NAME_LENGTH 500

void printConfig(const struct configtpl_ConfigParam *cfg, char *prefix)
{
    prefix = prefix ? prefix : "";
    const struct configtpl_Array_ConfigParam *vec;
    const struct configtpl_Array_ConfigParamDictItem *map;
    switch (cfg->param_type)
    {
        case CONFIGTPL_CONFIG_PARAM_TYPE_INT:
            printf("%s=%ld\n", prefix, cfg->value.integer);
            break;
        case CONFIGTPL_CONFIG_PARAM_TYPE_STRING:
            printf("%s=%s\n", prefix, cfg->value.string);
            break;
        case CONFIGTPL_CONFIG_PARAM_TYPE_BOOLEAN:
            printf("%s=%s\n", prefix, cfg->value.boolean ? "true" : "false");
            break;
        case CONFIGTPL_CONFIG_PARAM_TYPE_NULL:
            printf("%s=null\n", prefix);
            break;
        case CONFIGTPL_CONFIG_PARAM_TYPE_VEC:
            vec = &cfg->value.vector;
            for (int j = 0; j < vec->len; j++)
            {
                char fullNameSub[MAX_NAME_LENGTH];
                sprintf(fullNameSub, "%s[%d]", prefix, j);
                printConfig(&vec->data[j], fullNameSub);
            }
            break;
        case CONFIGTPL_CONFIG_PARAM_TYPE_MAP:
            map = &cfg->value.map;
            for (int j = 0; j < map->len; j++)
            {
                char fullNameSub[MAX_NAME_LENGTH];
                sprintf(fullNameSub, "%s.%s", prefix, map->data[j].name);
                printConfig(map->data[j].value, fullNameSub);
            }
            break;
    }
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        fprintf(stderr, "Usage: %s PATHS\n", argv[0]);
        return 1;
    }
    char *paths = argv[1];

    if (CONFIGTPL_SIMPLE_RESULT_ERROR == configtpl_init())
    {
        fprintf(stderr, "Failed to initialize configtpl");
        return 1;
    }

    configtpl_CfgBuilderHandle handle = configtpl_configbuilder_new();

    configtpl_ConstCharPtr context_data[2][2] = {
        {"mykey", "myval"},
        {"mykey2", "myval2"}
    };
    struct configtpl_Array_StringKV ctx = {
        .data = context_data,
        .len = 2
    };

    configtpl_ConstCharPtr overrides_data[1][2] = {};
    struct configtpl_Array_StringKV overrides = {
        .data = overrides_data,
        .len = 0
    };

    const struct configtpl_BuildResult *r = configtpl_configbuilder_build_from_files(handle, paths, &overrides, &ctx);
    int ret_code = -1;
    int line_nr = 0;
    char *line = NULL;
    switch (r->status)
    {
        case CONFIGTPL_BUILD_STATUS_SUCCESS:
            printConfig(&r->output, NULL);
            break;
        case CONFIGTPL_BUILD_STATUS_ERROR_INVALID_HANDLE:
            fprintf(stderr, "Invalid handle: %d", handle);
            break;
        case CONFIGTPL_BUILD_STATUS_ERROR_BUILDING:
            fprintf(stderr, "Failed to render template: %s\n", r->error_msg);
            break;
        default:
            fprintf(stderr, "Unknown error occurred.");
            break;

    }

    configtpl_configbuilder_result_free((struct configtpl_BuildResult*)r);
    configtpl_configbuilder_free(handle);

    if (CONFIGTPL_SIMPLE_RESULT_ERROR == configtpl_cleanup())
    {
        fprintf(stderr, "Failed to deinitialize configtpl");
        ret_code = 1;
    }

    return ret_code;
}
