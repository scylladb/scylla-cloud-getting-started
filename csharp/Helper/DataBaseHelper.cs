using Cassandra;

namespace MediaPlayer.Helper;

public static class DataBaseHelper
{
    public static bool RowSetHasResult(RowSet rowSet)
    {
        var rows = rowSet
            .GetRows()
            .ToList();

        return rows.Count > 0;
    }
}