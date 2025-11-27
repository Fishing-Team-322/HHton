#include <gtest/gtest.h>
#include <spdlog/spdlog.h>

#include "services/hackathon-config/src/repositories/HackathonRepository.h"

class StubDbPool : public DbPool {
public:
    StubDbPool() : DbPool("") {}
    std::unique_ptr<pqxx::connection> acquire() const override {
        throw std::runtime_error("No database available in test");
    }
};

TEST(HackathonRepositoryTest, CreateFailsWithoutDatabase) {
    StubDbPool pool;
    HackathonRepository repo(pool);

    Hackathon h{};
    h.organizer_id = 42;
    h.title = "Test";
    h.description = "Desc";
    h.format = "online";
    h.status = "draft";

    EXPECT_THROW(repo.create(h), std::exception);
}
