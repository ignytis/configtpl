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
            case CONFIGTPL_CONFIG_PARAM_TYPE_FLOAT:
                printf("%s=%lf\n", prefix, cfg->value.float_num);
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
    const char *paths_val[] = {
        argv[1],
    };
    struct configtpl_Array_ConstCharPtr paths = {
        .len = 1,
        .data = paths_val,
    };

    if (CONFIGTPL_SIMPLE_RESULT_ERROR == configtpl_init())
    {
        fprintf(stderr, "Failed to initialize configtpl");
        return 1;
    }

    configtpl_CfgBuilderHandle handle = configtpl_configbuilder_new();

    struct configtpl_ConfigParam context_item1 = {
        .param_type = CONFIGTPL_CONFIG_PARAM_TYPE_STRING,
        .value.string = "my_val1",
    };
    struct configtpl_ConfigParam context_item2 = {
        .param_type = CONFIGTPL_CONFIG_PARAM_TYPE_STRING,
        .value.string = "my_val2",
    };
    struct configtpl_ConfigParamDictItem context_items[2] = {
        {
            .name = "my_key1",
            .value = &context_item1,
        },
        {
            .name = "my_key2",
            .value = &context_item2,
        },
    };
    struct configtpl_ConfigParam context = {
        .param_type = CONFIGTPL_CONFIG_PARAM_TYPE_MAP,
        .value.map = {
            .data = context_items,
            .len = 2
        }
    };

    struct configtpl_ConfigParam overrides_item1 = {
        .param_type = CONFIGTPL_CONFIG_PARAM_TYPE_STRING,
        .value.string = "my_overidden_value",
    };
    struct configtpl_ConfigParamDictItem overrides_items[1] = {
        {
            .name = "my_overidden_key",
            .value = &overrides_item1,
        },
    };
    struct configtpl_ConfigParam overrides = {
        .param_type = CONFIGTPL_CONFIG_PARAM_TYPE_MAP,
        .value.map = {
            .data = overrides_items,
            .len = 1
        }
    };

    char *app_env_prefix = "MY_APP";
    struct configtpl_BuildArgs args = {
        .env_vars_prefix = app_env_prefix,
        .context = &context,
        .defaults = NULL,
        .paths = paths,
        .overrides = &overrides,
    };


    fprintf(stderr, "Before build...\n");
    const struct configtpl_BuildResult *r = configtpl_configbuilder_build(handle, args);
    fprintf(stderr, "After build...\n");
    int ret_code = -1;
    int line_nr = 0;
    char *line = NULL;
    switch (r->status)
    {
        case CONFIGTPL_BUILD_STATUS_SUCCESS:
            fprintf(stderr, "Operation succeeded. Here is the result:\n");
            printConfig(&r->output, NULL);
            ret_code = 0;
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

    configtpl_configbuilder_result_free(r);
    configtpl_configbuilder_free(handle);

    if (CONFIGTPL_SIMPLE_RESULT_ERROR == configtpl_cleanup())
    {
        fprintf(stderr, "Failed to deinitialize configtpl");
        ret_code = 1;
    }

    return ret_code;
}
