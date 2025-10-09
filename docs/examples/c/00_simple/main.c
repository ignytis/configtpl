#include <stdio.h>
#include <stdlib.h>

#include "../../../../include/configtpl.h"


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
            for (int i = 0; i < r->output.len; i++)
            {
                configtpl_StringKV *kv = &r->output.data[i];
                printf("%s=%s\n", (*kv)[0], (*kv)[1]);
            }
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

    configtpl_configbuilder_result_free((struct configtpl_BuildResult*)r);
    configtpl_configbuilder_free(handle);

    if (CONFIGTPL_SIMPLE_RESULT_ERROR == configtpl_cleanup())
    {
        fprintf(stderr, "Failed to deinitialize configtpl");
        ret_code = 1;
    }

    return ret_code;
}
