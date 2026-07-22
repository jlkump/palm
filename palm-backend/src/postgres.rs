

use postgres::Client;

use crate::model::service::Service;


impl Service for Client {
    fn create_user(&mut self, info: &crate::model::intermediate::UserCreation) -> impl Future {
        async {
            self.execute("
                    INSERT INTO Users (email, first_name, last_name, pass)
                    VALUES ($1, $2, $3, $4);
            ", &[&info.email, &info.first_name, &info.last_name, &info.password])
        }
    }

    fn sync_user(&mut self, info: &crate::model::intermediate::UserSync) -> impl Future {
        async {
            self.execute("
                UPDATE Users
                SET  email = $1, first_name = $2, last_name = $3, pass = $4, archived = $5, 
                WHERE user_id = $6
            ", &[&info.email, &info.first_name, &info.last_name, &info.password, &Box::new(info.archived), &info.service_user_id])
        }
    }

    fn remove_user(&mut self, info: &crate::model::intermediate::UserDelete)  -> impl Future {
        async {
         self.execute("
            DELETE FROM Users
            WHERE $1
        ", &[&info.service_user_id])
        }
    }
}

