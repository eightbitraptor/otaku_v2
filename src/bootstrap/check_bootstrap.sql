SELECT (EXISTS(SELECT name
               FROM sqlite_master
               WHERE tbl_name = 'images' AND type = 'table'
        )
        & (SELECT IFNULL(MAX(id), 0)
           FROM 'schema_versions'
           WHERE EXISTS(SELECT name
                        FROM sqlite_master
                        WHERE tbl_name = 'schema_versions'))) AS bootstrapped
