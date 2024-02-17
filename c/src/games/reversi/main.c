#include "arbor.h"
#include "cli.h"
#include "reversi.h"

int main (int argc, char* argv[])
{
    Arbor_Game_Interface ifc = {
        .actions    = reversi_actions,
        .copy       = reversi_copy,
        .delete     = reversi_delete,
        .make       = reversi_make,
        .eval       = reversi_eval,
        .side       = reversi_side,
        .show       = reversi_show,
        .vector     = reversi_vector,
        .prob       = reversi_prob
    };
    Arbor_Game game = reversi_new();
    int result = cli(game, &ifc, argc, argv);

    reversi_delete(game);

    return result;
}
