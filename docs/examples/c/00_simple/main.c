#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "../../../../include/configtpl.h"


int main(int argc, char** argv)
{
    if (argc < 2)
    {
        fprintf(stderr, "Usage: %s TEMPLATE\n", argv[0]);
        return 1;
    }
    char *tpl = argv[1];
    
    configtpl_EnrironmentHandle env_handle = configtpl_new_environment();

    configtpl_ConstCharPtr context_data[2][2] = {
        {"mykey", "myval"},
        {"mykey2", "myval2"}
    };
    struct configtpl_Array__________ConstCharPtr__________2 ctx = {
        .len = 2,
        .data = context_data
    };

    struct configtpl_RenderResult* r = configtpl_render(env_handle, tpl, &ctx);
    int ret_code = -1;
    int line_nr = 0;
    char *line = NULL;
    switch (r->status)
    {
        case CONFIGTPL_RENDER_STATUS_SUCCESS:
            printf("%s", r->output);
            ret_code = 0;
            break;
        case CONFIGTPL_RENDER_STATUS_ERROR_INVALID_HANDLE:
            fprintf(stderr, "Invalid handle: %d", env_handle);
            break;
        case CONFIGTPL_RENDER_STATUS_ERROR_TEMPLATE_RENDER:
            fprintf(stderr, "Failed to render template: %s (line %ld, %ld - %ld)\n", r->output, r->location.line, r->location.start, r->location.end);
            line = strtok(tpl, "\n");
            while (NULL != line)
            {
                line_nr++;
                if (line_nr < r->location.line)
                {
                    line = strtok(NULL, "\n");
                    continue;
                };
                // Print the error line itself
                fprintf(stderr, "  %s\n", line);
                for (int i = 0; i <= r->location.start; i++)
                {
                    fprintf(stderr, " ");
                }

                for (int i = r->location.start; i < r->location.end; i++)
                {
                    fprintf(stderr, "^");
                }
                fprintf(stderr, "\n");
                
                break;
            }
            // Print the error message which points to place in template where the erorr occurred.
            // Use tpl as template string and `r` to locate the line and position of error

            break;
        default:
            fprintf(stderr, "Unknown error occurred.");
            break;
    
    }

    configtpl_render_free_result(r);
    configtpl_free_environment(env_handle);

    return ret_code;
}